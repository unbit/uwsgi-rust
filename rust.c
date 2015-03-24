#include <uwsgi.h>

int rust_request_handler(struct wsgi_request *);
int rust_add_environ(void *, char *, uint16_t, char *, uint16_t);
int rust_load_fn(char *, uint16_t);

extern struct uwsgi_server uwsgi;
struct uwsgi_plugin rust_plugin;

struct uwsgi_rust {
	// function to call in the current address space
	char *fn;
} urust;

static struct uwsgi_option rust_options[] = {
	{"rust-fn", required_argument, 0, "rust function to call at every request", uwsgi_opt_set_str, &urust.fn, 0},
	UWSGI_END_OF_OPTIONS
};

static void rust_apps() {
	if (!urust.fn) return;

	if (rust_load_fn(urust.fn, strlen(urust.fn))) {
		uwsgi_log("[rust] unable to find function \"%s\"\n", urust.fn);
		exit(1);
	}

	time_t now = uwsgi_now();
	int id = uwsgi_apps_cnt;

	struct uwsgi_app *ua = uwsgi_add_app(id, rust_plugin.modifier1, "", 0, NULL, NULL);
        if (!ua) {
                uwsgi_log("[rust] unable to mount app\n");
                exit(1);
        }

        ua->responder0 = urust.fn;
        ua->responder1 = urust.fn;
        ua->started_at = now;
        ua->startup_time = uwsgi_now() - now;
        uwsgi_log("Rust app/mountpoint %d loaded in %d seconds\n", id, (int) ua->startup_time);

	uwsgi_emulate_cow_for_apps(id);
}

int uwsgi_rust_build_environ(struct wsgi_request *wsgi_req, void *hm) {
	int i;
	for(i=0;i<wsgi_req->var_cnt;i++) {
		char *key = (char *)wsgi_req->hvec[i].iov_base;
		uint16_t key_len = wsgi_req->hvec[i].iov_len;
		char *val = (char *)wsgi_req->hvec[i+1].iov_base;
		uint16_t val_len = wsgi_req->hvec[i+1].iov_len;
		if (rust_add_environ(hm, key, key_len, val, val_len)) return -1;
                i++;
        }

	return 0;
}

static int rust_request(struct wsgi_request *wsgi_req) {

        if (!wsgi_req->uh->pktsize) {
                uwsgi_log("Empty request. skip.\n");
                return -1;
        }

        if (uwsgi_parse_vars(wsgi_req)) {
                return -1;
        }

	wsgi_req->app_id = uwsgi_get_app_id(wsgi_req, wsgi_req->appid, wsgi_req->appid_len, rust_plugin.modifier1);
	if (wsgi_req->app_id == -1 && !uwsgi.no_default_app && uwsgi.default_app > -1) {
                if (uwsgi_apps[uwsgi.default_app].modifier1 == rust_plugin.modifier1) {
                        wsgi_req->app_id = uwsgi.default_app;
                }
        }

        if (wsgi_req->app_id == -1) {
                uwsgi_404(wsgi_req);
                return UWSGI_OK;
        }

	uwsgi_log("app id = %d %p\n", wsgi_req->app_id, wsgi_req);

	return rust_request_handler(wsgi_req);
}

struct uwsgi_plugin rust_plugin = {
	.name = "rust",
	.modifier1 = 0,
	.options = rust_options,
	.init_apps = rust_apps,
	.request = rust_request,
	.after_request = log_request,
};

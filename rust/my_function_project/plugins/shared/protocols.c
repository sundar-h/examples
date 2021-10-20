int init(const char* config);
int finalize();
const char* language_type();
const char* plugin_type();
const char* name();
const char* info();
int send(const char* payload);
const char* next();

// 传递插件配置元信息
typedef struct {
    char *name;
    char *lang_type;
    char *plugin_type;
} Config;
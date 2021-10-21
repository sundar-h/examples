const char* init(const char* config);
const char* finalize();
const char* language_type();
const char* plugin_type();
const char* name();
const char* info();

const char* send(const char* payload);
const char* next();

// 传递插件配置元信息
typedef struct {
    char *name;
    char *lang_type;
    char *plugin_type;
} Config;
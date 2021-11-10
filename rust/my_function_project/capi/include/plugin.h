#ifndef _PLUGIN_H
#define _PLUGIN_H

typedef struct PluginInfo {
    char *name;
    char *language;
    char *plugin_type;
    char *version;
} PluginInfo;


extern "C" {
const char *version(void);
} // extern "C"

#endif /* _PLUGIN_H */

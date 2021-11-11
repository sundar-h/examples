#ifndef _PLUGIN_H
#define _PLUGIN_H

typedef struct PluginInfo {
    char *name;
    char *language;
    char *plugin_type;
    char *version;
} PluginInfo;



#ifdef __cplusplus
extern "C" {
#endif // __cplusplus
const char *version(void);
#ifdef __cplusplus
} // extern "C"

#endif /* _PLUGIN_H */

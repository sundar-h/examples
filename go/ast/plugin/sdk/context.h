
// 不传递所有权
typedef struct Context {
    char *RequestID;
	int InitializerTimeout; // 初始化超时时间
	int Timeout;            // Open 函数运行超时时间
	char *Payload;             // 参数
} Context;


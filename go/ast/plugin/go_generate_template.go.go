package main

type Context struct {
	RequestID string // uuid
}

type Function struct {
	Name                  string // 函数名
	Handler               string // 函数计算的入口函数 文件名+_函数名 eg: main.Runmain.go 的Initializer函数
	Memory                uint   // 内存限制
	Timeout               uint   // 超时时间
	Initializer           string // 初始化函数(执行早于入口函数), 保证仅成功执行一次; 将数据库场景下连接池构建、函数依赖库加载等耗时长的业务逻辑放到Initializer函数中，避免每次运行函数都会做重复的操作，降低函数延时。
	initializationTimeout uint   // 初始化时间限制
}

type CustomPlugin struct {
}

func (p *CustomPlugin) Name() string {
	return "mqtt"
}

func (p *CustomPlugin) Open(config string) error {
	return nil
}

func (p *CustomPlugin) Close() error {
	return nil
}

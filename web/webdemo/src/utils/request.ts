import axios from 'axios'

const request = axios.create({
    baseURL: 'http://localhost:8101',
    timeout: 1000,
})

// 错误分为两种:
// 1. 4xx 5xx 错误
// 2. 业务错误

request.interceptors.response.use(
    (response) => {
        console.info('received response: ')
        console.table(response)

        const { data } = response
        return data
    },
    (err) => {
        if (axios.isAxiosError(err)) {
            console.log('AxiosError: ', err.toJSON())
        } else {
            console.log('unexpected error: ', err)
        }
    }
)

export interface Response<T = any> {
    data: T
    code: number
    message: string
}

// interface GrpcError {
//     message: string
//     code: number
// }

export default request

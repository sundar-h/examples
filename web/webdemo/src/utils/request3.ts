import axios, { AxiosRequestConfig } from 'axios'

const instance = axios.create({
    baseURL: '',
    timeout: 1000,
})

const request = async <T = any>(config: AxiosRequestConfig): Promise<T> => {
    // return
    try {
        const { data } = await instance.request<T>(config)
        console.error('got response: ', data)
        return data
    } catch (err) {
        if (axios.isAxiosError(err)) {
            console.log('is axios error')
        }
        console.error('got error: ', err)
        return err
    }
}

export default request

import axios from 'axios'
import { CallReply } from '../gen/invoke'
import request from '../utils/request'

export const invoke = () => {
    return request.get<CallReply>('/v1/example/invoke', {
        params: {
            name: 'hello World',
        },
    })
}

const invoke2 = async () => {
    try {
        const {data} = await request.get<CallReply>("/v1/example/invoke", {
            params: {
                name: 'hello World',
            },
        })
        return data.message
    } catch (err) {
        // Promise.reject(err)
        throw err
    }
}

import axios from 'axios'
import Cookies from 'js-cookie'
import { ElMessage } from 'element-plus'

// 时间过期的设置
const TIME_OUT = 300000
const TIPS_DURATION = 2
const http = axios.create({
    timeout: TIME_OUT
})
/**
 * @description 发送请求的拦截器,为请求增加时间戳
 */
http.interceptors.request.use(function (config) {
    // 这里设置 header的默认传递的参数
    let headers = {
        'Accept': 'application/json,text/plain,*/*',
        'Content-Type': 'application/json'
    }
    config.headers = Object.assign(headers, config.headers)
    if (config.method.toLocaleLowerCase() === 'get') {
        if (!config.params) {
            Object.assign(config, {
                params: {
                    _: new Date().getTime()
                }
            })
        } else {
            Object.assign(config.params, {
                _: new Date().getTime()
            })
        }
    }
    return config
}, function (error) {
    return Promise.reject(error)
})
/**
 * @description 返回的请求拦截器
 */
http.interceptors.response.use(async (response) => {
    const { status, data } = response
    // 证明接口正常
    if (status === 200) {
        // handle normal error
        if (!data.success) {
            ElMessage.error(data.message)
            return Promise.reject(data)
        }
        // 如有有payload 就直接返回 data 不然返回 response
        return data.payload !== undefined ? data : response
    }
    return response
}, error => {
    let res = error?.response?.data
    // 未登录处理
    if (res && res.code == 401) {
        ElMessage.error("Session过期，请重新登录..")
        Cookies.remove("sid")
        setTimeout(() => {
            location.reload()
        }, 1500)
    } else {
        ElMessage.error(res?.message || error.message)
        return Promise.reject(error)
    }
})

const get = (url, params, timeout, headers = {}) => {
    return http({
        method: 'get',
        url: url,
        params: params,
        timeout: timeout || TIME_OUT,
        headers
    })
}
const post = (url, params, timeout, headers = {}) => {
    return http({
        method: 'post',
        url: url,
        data: params,
        timeout: timeout || TIME_OUT,
        headers
    })
}
const put = (url, params, headers = {}) => {
    return http({
        method: 'put',
        url: url,
        data: params,
    })
}
const deleteReq = (url, params, headers = {}) => {
    return http({
        method: 'delete',
        url: url,
        data: params,
        headers
    })
}

/** 导出各种类型的请求及原始的请求 */
export {
    http as axios,
    post,
    get,
    put,
    deleteReq
}

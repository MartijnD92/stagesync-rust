import { AxiosRequestConfig } from 'axios'
import { ApiResponse } from '../models/api-response'
import { callExternalApi } from './external-api.service'

const apiServerUrl = import.meta.env.REACT_APP_API_SERVER_URL

export const getPublicResource = async (): Promise<ApiResponse> => {
    const config: AxiosRequestConfig = {
        url: `${apiServerUrl}/api/v1/artists`,
        method: 'GET',
        headers: {
            'content-type': 'application/json',
        },
    }

    const { data, error } = (await callExternalApi({ config })) as ApiResponse

    return {
        data,
        error,
    }
}

export const getUserDashboard = async (
    accessToken: string
): Promise<ApiResponse> => {
    const config: AxiosRequestConfig = {
        url: `${apiServerUrl}/api/v1/gigs`,
        method: 'GET',
        headers: {
            'content-type': 'application/json',
            Authorization: `Bearer ${accessToken}`,
        },
    }

    const { data, error } = (await callExternalApi({ config })) as ApiResponse

    return {
        data,
        error,
    }
}

export const getAdminResource = async (
    accessToken: string
): Promise<ApiResponse> => {
    const config: AxiosRequestConfig = {
        url: `${apiServerUrl}/api/v1/users`,
        method: 'GET',
        headers: {
            'content-type': 'application/json',
            Authorization: `Bearer ${accessToken}`,
        },
    }

    const { data, error } = (await callExternalApi({ config })) as ApiResponse

    return {
        data,
        error,
    }
}

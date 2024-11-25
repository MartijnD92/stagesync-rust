import { AxiosRequestConfig } from 'axios'
import { ArtistResponse, UserResponse } from '../models/api-response'
import callExternalApi from './external-api.service'
import { formatArtist } from '../services/helpers/mappings'

const apiServerUrl = __API_SERVER_URL__

// export const getPublicResource = async (): Promise<ApiResponse> => {
//     const config: AxiosRequestConfig = {
//         url: `${apiServerUrl}/api/v1/artists`,
//         method: 'GET',
//         headers: {
//             'content-type': 'application/json',
//         },
//     }

//     const { data, error } = (await callExternalApi({ config })) as ApiResponse

//     return {
//         data,
//         error,
//     }
// }

export const getArtistData = async (
    accessToken: string
): Promise<ArtistResponse> => {
    const config: AxiosRequestConfig = {
        url: `${apiServerUrl}/api/v1/artists/me`,
        method: 'GET',
        headers: {
            'content-type': 'application/json',
            Authorization: `Bearer ${accessToken}`,
        },
    }

    const { data, error } = await callExternalApi<'artist'>({
        config,
    })

    return {
        data: data?.map(formatArtist) || null,
        error,
    }
}

export const getArtistDataAdmin = async (
    accessToken: string
): Promise<ArtistResponse> => {
    const config: AxiosRequestConfig = {
        url: `${apiServerUrl}/api/v1/artists`,
        method: 'GET',
        headers: {
            'content-type': 'application/json',
            Authorization: `Bearer ${accessToken}`,
        },
    }

    const { data, error } = await callExternalApi<'artist'>({
        config,
    })

    return {
        data: data?.map(formatArtist) || null,
        error,
    }
}

export const getProfileData = async (
    accessToken: string
): Promise<UserResponse> => {
    const config: AxiosRequestConfig = {
        url: `${apiServerUrl}/api/v1/users/me`,
        method: 'GET',
        headers: {
            'content-type': 'application/json',
            Authorization: `Bearer ${accessToken}`,
        },
    }

    const { data, error } = await callExternalApi<'user'>({
        config,
    })

    return {
        data,
        error,
    }
}

import React from 'react'
import { AppError } from '../models/app-error'
import {
    ArtistResponse,
    GigResponse,
    UserResponse,
    ApiResponse,
} from '../models/api-response'
import { ArtistFormatted } from '../models/artist'
import { Gig } from '../models/gig'
import { User } from '../models/user'
import { useAuth0 } from '@auth0/auth0-react'
import {
    getArtistData,
    // getGigData,
    getProfileData,
} from '../services/data.service'

type DataType = {
    artist: ArtistFormatted
    gig: Gig
    user: User
}

type ResponseType = {
    artist: ArtistResponse
    gig: GigResponse
    user: UserResponse
}

function useData<T extends keyof DataType>(type: T): [DataType[T][], AppError] {
    const [data, setData] = React.useState<DataType[T][]>([])
    const [error, setError] = React.useState<AppError | null>(null)

    const { getAccessTokenSilently } = useAuth0()

    const fetchFunction = React.useMemo(() => {
        switch (type) {
            case 'artist':
                return getArtistData
            // case 'gig':
            //     return getGigData
            case 'user':
                return getProfileData
        }
    }, [type])

    React.useEffect(() => {
        let isMounted = true

        const fetchData = async () => {
            try {
                const accessToken = await getAccessTokenSilently()
                const response: ApiResponse[T] = (await fetchFunction?.(
                    accessToken
                )) as ResponseType[T]

                if (!isMounted) return

                if (response.data) {
                    if (Array.isArray(response.data)) {
                        setData(response.data as DataType[T][])
                    } else {
                        setData([response.data] as DataType[T][])
                    }
                }
                if (response.error) {
                    setError(response.error)
                }
            } catch {
                setError({ message: 'Failed to fetch data' })
            }
        }

        fetchData()

        return () => {
            isMounted = false
        }
    }, [getAccessTokenSilently, fetchFunction])

    return [data, error]
}

export default useData

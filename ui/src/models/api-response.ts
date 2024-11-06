import { AppError } from './app-error'
import { ArtistFormatted } from './artist'
import { Gig } from './gig'
import { User } from './user'

export interface ApiResponse {
    artist: ArtistResponse
    gig: GigResponse
    user: UserResponse
}

export interface ArtistResponse {
    data: ArtistFormatted[] | null
    error: AppError | null
}
export interface GigResponse {
    data: Gig[] | null
    error: AppError | null
}
export interface UserResponse {
    data: User[] | null
    error: AppError | null
}

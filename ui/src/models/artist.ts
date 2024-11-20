import { Gig } from './gig'
import { User } from './user'

export interface Artist {
    id: string
    name: string
    description?: string
    image?: string
    fee: number
    currency: string
    genre?: string
    location?: string
    owner: User
    gigs: Gig[]
    created_at: string
    updated_at: string
}

export interface ArtistFormatted extends Artist {
    owner_name: string
    fee_with_currency: string
    next_gig: string
}

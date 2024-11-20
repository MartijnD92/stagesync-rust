import { Artist, ArtistFormatted } from '../../models/artist'

export function formatArtist(artist: Artist): ArtistFormatted {
    const gigs = artist.gigs.map((gig) => {
        return {
            ...gig,
            date: new Date(gig.date).toLocaleDateString('nl-NL'),
        }
    })

    return {
        ...artist,
        owner_name: `${artist.owner.first_name} ${artist.owner.last_name}`,
        fee_with_currency: `${artist.currency} ${artist.fee.toLocaleString(
            'nl-NL',
            {
                minimumFractionDigits: 2,
            }
        )}`,
        gigs,
        next_gig: gigs[0] ? `${gigs[0]?.date} / ${gigs[0]?.location}` : '',
    }
}
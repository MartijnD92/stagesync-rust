import { Artist, ArtistFormatted } from '../../models/artist'
export function formatArtist(a: Artist): ArtistFormatted {
    const gigs = a.gigs.map((g) => {
        return {
            ...g,
            date: new Date(g.date).toLocaleDateString('nl-NL'),
        }
    })

    return {
        ...a,
        ownerName: `${a.owner.first_name} ${a.owner.last_name}`,
        feeWithCurrency: `${a.currency} ${a.fee.toLocaleString('nl-NL', {
            minimumFractionDigits: 2,
        })}`,
        gigs,
    }
}

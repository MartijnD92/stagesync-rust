import { NavLink } from 'react-router-dom'
import { IoMdMusicalNote as NoteIcon } from 'react-icons/io'
import s from './SideBar.module.css'

function SideBar({ componentClass }: { componentClass?: string }) {
    const links = [
        { icon: '', label: 'Dashboard', href: '/' },
        { icon: '', label: 'Bookings', href: '/bookings' },
        { icon: `${NoteIcon}`, label: 'Artists', href: '/artists' },
        { icon: '', label: 'My Profile', href: '/profile' },
        { icon: '', label: 'Settings', href: '/settings' },
    ]

    return (
        <nav className={`${componentClass} ${s.wrapper}`}>
            <ul className={s.buttons}>
                {links.map((l) => {
                    return (
                        <li className={s.button}>
                            {l.icon}
                            <NavLink
                                className={({ isActive }) =>
                                    isActive ? `${s.link} ${s.active}` : s.link
                                }
                                to={l.href}
                            >
                                {l.label}
                            </NavLink>
                        </li>
                    )
                })}
            </ul>
        </nav>
    )
}

export default SideBar

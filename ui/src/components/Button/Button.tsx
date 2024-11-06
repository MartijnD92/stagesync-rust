import s from './Button.module.css'

interface Props {
    children: string
    extraStyles: string
    onClickHandler: React.MouseEventHandler
}

function Button({ children, extraStyles, onClickHandler }: Props) {
    return (
        <button
            className={`${s.button} ${extraStyles}`}
            onClick={onClickHandler}
        >
            {children}
        </button>
    )
}

export default Button

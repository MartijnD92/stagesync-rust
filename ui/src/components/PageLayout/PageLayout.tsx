import styles from './PageLayout.module.css'

interface Props {
    children: JSX.Element
}

function PageLayout({ children }: Props) {
    return (
        <div className={styles.wrapper}>
            {/* <NavBar />
      <MobileNavBar /> */}
            <div className={styles.content}>{children}</div>
            {/* <PageFooter /> */}
        </div>
    )
}
export default PageLayout

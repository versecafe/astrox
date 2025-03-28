import * as React from 'react'

export function DisplayServerTime(): React.JSX.Element {
  const [serverTime, setServerTime] = React.useState<string>('')

  React.useEffect(() => {
    async function fetchServerTime() {
      try {
        const response = await fetch(
          `http://${import.meta.env.PUBLIC_HOST ?? 'localhost:3000'}/api/time/`,
        )
        // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment -- we trust the server here
        const data: [{ message: string }] = await response.json()
        setServerTime(data[0].message)
      } catch (error) {
        console.error('Failed to fetch server time:', error)
      }
    }

    fetchServerTime().catch((error) =>
      console.error('Failed to fetch server time:', error),
    )

    const intervalId = setInterval(fetchServerTime, 1000)

    return () => clearInterval(intervalId)
  }, [])

  return (
    <h1 className="font-display text-3xl">
      Server Time: {serverTime ?? 'No Time?!'}
    </h1>
  )
}

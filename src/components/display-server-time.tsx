import * as React from 'react'

export function DisplayServerTime(): JSX.Element {
  const [serverTime, setServerTime] = React.useState<string>('')

  React.useEffect(() => {
    const intervalId = setInterval(async () => {
      try {
        const response = await fetch('http://localhost:3000/api/time/')
        // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment -- we trust the server here
        const data: [{ message: string }] = await response.json()
        setServerTime(data[0].message)
      } catch (error) {
        console.error('Failed to fetch server time:', error)
      }
    }, 1000)

    return () => clearInterval(intervalId)
  }, [])

  return (
    <h1 className="font-display text-3xl">
      Server Time: {serverTime ?? 'No Time?!'}
    </h1>
  )
}

export function formatHms (input: number): string {
  const { seconds, minutes, hours } = hms(input)
  const minutesAndSeconds = `${String(minutes).padStart(2, '0')}:${String(seconds).padStart(2, '0')}`
  if (hours > 0) {
    return `${hours}:${minutesAndSeconds}`
  }
  return minutesAndSeconds

}

export interface Hms {
  seconds: number
  minutes: number
  hours: number
}

export function hms (input: number): Hms {
  const seconds = input % 60
  const minutes = ((input - seconds) / 60) % 60
  const hours = (input - seconds - (minutes * 60)) / 3600
  return { seconds, minutes, hours }
}

export function formatHumane (duration: number): string {
  const { hours, minutes } = hms(duration)
  const minString = minutes === 0 ? '' : `${minutes}min`
  const hourString = hours === 0 ? '' : `${hours}h`
  return `${hourString} ${minString}`.trim()
}

const formatter = new Intl.DateTimeFormat()

export function episodeDate (dateStr: string): string {
  const date = new Date(dateStr)
  return formatter.format(date)
}

export function ratio (a: number, b: number): string {
  if (b === 0) {
    return '0'
  }
  return `${(a / b) * 100}%`
}

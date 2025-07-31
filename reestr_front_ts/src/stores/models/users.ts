export interface User {
  id: number | null
  login: string
  username: string
  role: string | null
  is_active: boolean | false
}
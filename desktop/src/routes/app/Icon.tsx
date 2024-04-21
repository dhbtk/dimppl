import React from 'react'

export const Icon: React.FC<{ icon: string }> = ({ icon }) => {
  return (
    <span className="material-icons-outlined">{icon}</span>
  )
}

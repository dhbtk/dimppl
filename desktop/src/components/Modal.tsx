import React, { PropsWithChildren, useEffect, useRef, useState } from 'react'
import styled from 'styled-components'

interface ModalProps {
  isOpen: boolean
  onClose?: () => void
}

const StyledModal = styled.dialog`
  padding: 16px;
  border-radius: 9px;
  background-color: #F2F2F2;
  box-shadow: 8px 8px 16px 8px var(--gray10);
  width: 40%;
`

export const Modal: React.FC<PropsWithChildren<ModalProps>> = ({ children, isOpen, onClose }) => {
  const [isModalOpen, setModalOpen] = useState(isOpen)
  const modalRef = useRef<HTMLDialogElement | null>(null)

  useEffect(() => {
    setModalOpen(isOpen)
  }, [isOpen])
  useEffect(() => {
    const modalElement = modalRef.current
    if (modalElement) {
      if (isModalOpen) {
        modalElement.showModal()
      } else {
        modalElement.close()
      }
    }
  }, [isModalOpen])
  const handleCloseModal = () => {
    if (onClose) {
      onClose()
    }
    setModalOpen(false)
  }

  useEffect(() => {
    const modalElement = modalRef.current
    if (modalElement) {
      modalElement.addEventListener('close', () => {
        handleCloseModal()
      })
    }
  }, [modalRef, setModalOpen])

  return (
    <StyledModal ref={modalRef}>
      {children}
    </StyledModal>
  )
}

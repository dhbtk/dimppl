import React, { PropsWithChildren, useEffect, useRef, useState } from 'react'

interface ModalProps {
  isOpen: boolean
  onClose?: () => void
}

export const Modal: React.FC<PropsWithChildren<ModalProps>> = ({ children, isOpen, onClose }) => {
  const [isModalOpen, setModalOpen] = useState(isOpen)
  const modalRef = useRef<HTMLDialogElement | null>(null);

  useEffect(() => {
    setModalOpen(isOpen);
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
      onClose();
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
    <dialog ref={modalRef}>
      {children}
    </dialog>
  )
}

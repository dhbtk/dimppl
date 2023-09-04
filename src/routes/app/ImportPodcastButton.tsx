import React, { useState } from 'react'
import { useMutation, useQueryClient } from '@tanstack/react-query'
import { podcastApi } from '../../backend/podcastApi.ts'
import { Modal } from '../../components/Modal.tsx'
import { ToolbarButton } from './ToolbarButton.tsx'

export const ImportPodcastButton: React.FC = () => {
  const [open, setOpen] = useState(false)
  const [podcastUrl, setPodcastUrl] = useState('')
  const [errorMsg, setErrorMsg] = useState('')
  const queryClient = useQueryClient()
  const mutation = useMutation({
    mutationFn: podcastApi.importPodcast,
    onSuccess: () => {
      setPodcastUrl('')
      setOpen(false)
      queryClient.invalidateQueries({ queryKey: ['allPodcasts']})
    },
    onError: error => {
      setErrorMsg((error as any).toString())
    }
  })
  return (
    <>
      <ToolbarButton type="button" onClick={() => setOpen(true)}>
        <span className="material-icons-outlined">add</span>
      </ToolbarButton>
      <Modal
        isOpen={open}
        onClose={() => setOpen(false)}
        >
        <div>
          <p>Insira a URL do feed RSS do podcast.</p>
          <input type="text" value={podcastUrl} onChange={(e) => setPodcastUrl(e.target.value)}/>
          <p>{errorMsg}</p>
          <div style={{ display: 'flex' }}>
            <button type="button" onClick={() => setOpen(false)} style={{ marginLeft: 'auto' }}>
              Cancelar
            </button>
            <button type="button" disabled={mutation.isLoading} onClick={() => mutation.mutate(podcastUrl)} style={{ marginLeft: 'auto' }}>
              Adicionar
            </button>
          </div>
        </div>
      </Modal>
    </>
  )
}

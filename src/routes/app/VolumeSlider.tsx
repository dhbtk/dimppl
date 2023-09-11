import React, { useCallback, useEffect, useState } from 'react'
import styled from 'styled-components'
import { Icon } from './Icon.tsx'
import { configApi } from '../../backend/configApi.ts'

const Container = styled.div`
  display: flex;
  align-items: center;
`

const SmallActionButton = styled.button`
  cursor: default;
  color: rgb(189, 189, 189);
  margin-top: 3px;
  
  & > span.material-icons-outlined {
    font-size: 18px;
  }
  
  &:hover {
    color: rgb(128, 128, 128);
  }
`

const VolumeSliderInput = styled.input<{ width: string }>`
  -webkit-appearance: none;
  height: 3px;
  border-radius: 2px;
  background: rgb(242, 242, 242);
  background-image: linear-gradient(rgb(189, 189, 189), rgb(189, 189, 189));
  background-size: ${props => props.width} 100%;
  background-repeat: no-repeat;
  width: 100%;
  
  &::-webkit-slider-thumb {
    -webkit-appearance: none;
    height: 13px;
    width: 13px;
    border-radius: 50%;
    background: #fff;
    border: 1px solid rgb(192, 192, 192);
  }
  
  &::-webkit-slider-runnable-track {
    -webkit-appearance: none;
    box-shadow: none;
    border: none;
    background: transparent;
  }
`

export const VolumeSlider: React.FC = () => {
  const [volume, setVolume] = useState(1.0)
  const submitVolume = useCallback(async (newVolume: number) => {
    setVolume(newVolume)
    await configApi.setVolume(newVolume);
  }, [setVolume])
  useEffect(() => {
    configApi.load().then(config => setVolume(config.volume))
  }, [])
  return (
    <Container>
      <SmallActionButton onClick={() => submitVolume(0)}><Icon icon="volume_mute"/></SmallActionButton>
      <VolumeSliderInput
        type="range"
        min={0}
        max={1}
        width={`${(volume)*100}%`}
        step={0.01}
        value={volume}
        onChange={(e) => submitVolume(e.target.valueAsNumber)}
      />
      <SmallActionButton onClick={() => submitVolume(1)}><Icon icon="volume_up"/></SmallActionButton>
    </Container>
  )
}

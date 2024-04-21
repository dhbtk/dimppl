import styled from 'styled-components'

export const WrapperDiv = styled.div`
  display: flex;
  flex-direction: column;
  padding: 0 12px;
  flex: 1;
  gap: 12px;
`
export const Title = styled.h1`
  font-size: 36px;
`
export const RadioGroup = styled.div`
  display: flex;
  flex-direction: column;
  gap: 3px;
  padding-left: 12px;

  & > label {
    display: flex;
    align-items: center;
    gap: 6px;
  }
`
export const AccessKeyGroup = styled.div`
  max-width: 80%;
  display: flex;
  flex-direction: column;
  gap: 3px;
  padding-left: 12px;

  & > input {
    padding: 3px;
    border-radius: 3px;
    border: 1px solid var(--gray05);
  }
`

import React, { Component } from 'react'
import { connect } from 'react-redux'

class Home extends Component {
  constructor(props) {
    super(props)

    this.state = {
      success: '',
      failure: '',
      modalOpen: false,
      hash: '',
      data: '',
      loading: false
    }

    this.handleChange = this.handleChange.bind(this)
    this.handleSubmit = this.handleSubmit.bind(this)
  }

  handleChange(event) {
    const value = event.target.value ? event.target.value : ''
    
    this.setState({
      [event.target.name]: value
    })
  }

  handleSubmit(event) {
    event.preventDefault()

    this.setState({
      loading: true
    })

    if (this.state.hash !== '') {
      this.props.ipfs.catJSON(this.state.hash, async (err, data) => {
        if(err) {
          // console.log(err)
          this.setState({
            modalOpen: true,
            failure: `Error occured: ${err.message}`
          })
        } else {
          this.setState({
            data: data
          })
        }
      })
    } else {
      this.setState({
        modalOpen: true,
        failure: `You need to enter IPFS hash.`
      })
    }

    this.setState({
      loading: false
    })
    }

    download() {            //ToDo: add file name and type
        fetch('https://ipfs.infura.io:5001/api/v0/cat/QmXBdBt7MmeVAVmzM97hhGiHyZwkaWX2MkhR7dd64B5YMf')
            .then(response => {
                response.blob().then(blob => {
                    let url = window.URL.createObjectURL(blob);
                    let a = document.createElement('a');
                    a.href = url;
                    a.download = 'download.png';        //make 'download.png' a variable with file name
                    a.click();
                });
                //window.location.href = response.url;
            });
    }

  render() {
    return (
      <div>
            <div align="center">
          <h1 align="center">Load data from IPFS</h1>
          <form onSubmit={this.handleSubmit}>
                    <div pad='small' align='center'>
              <label labelfor="hash">Enter IPFS hash:</label>
                    </div>
                    <div pad='small' align='center'>
                        <input  id='hash'
                type='text'
                name='hash'
                onChange={this.handleChange}
                value={this.state.hash}
                placeholder='E.g. QmfWyGyMYHqqzEFUmfoUJyPQ6EzGnoB18v9CNbPjczXGgH' />
                    </div>
                    <div pad='small' align='center'>
                        {this.state.loading ? 'Loading...' : <button primary={true} type='submit' label='Get it'>Get it</button> }
                    </div>
                    <div pad='small' align='center'>
              <label align="cenyer">{ this.state.hash ? `Hash: ${this.state.hash}` : '' }</label>
                    </div>
          </form>
          { this.state.data ? <img src={this.state.data} size='large' align="center" />
            : ''
          }
                <div align="center">
            <label align="center">
              If you want to add this image as your image source, use this url:
            </label>
            <pre>
              https://ipfs.infura.io:5001/api/v0/cat/{this.state.hash}
            </pre>
                </div>
            </div>
            <h3>Download File using Button</h3>
            <button onClick={this.download}>Download</button>
            <p />
            <h3>Download File using Link</h3>
            <img src={this.state.data} size='large' align="center" />
            <a href="#" onClick={this.download}>Download</a>
            <a href='https://ipfs.infura.io:5001/api/v0/cat/QmWwvxqHtcF93LRinEPjHzMzE1WCHsjBrv2Srzd8tQVRHr' download>Click to download</a>
          { this.state.modalOpen && <toast
            status={this.state.success ? 'ok' : 'critical' }>
              <p>{ this.state.success ? this.state.success : null }</p>
              <p>{ this.state.failure ? this.state.failure : null }</p>
            </toast>
          }
        </div>
    )
  }
}

function mapStateToProps(state) {
  return {
    ipfs: state.ipfs
  }
}

export default connect(mapStateToProps)(Home)

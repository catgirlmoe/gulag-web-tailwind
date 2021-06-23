/*
//  Copyright 2021 catgirl.moe contributors <https://catgirl.moe>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

module.exports = {
  purge: [
    "./templates/**/*.html"
  ],
  theme: {
    extend: {
      colors: {
        'white': '#ffffff',
        'light': {
          DEFAULT: '#f7f7f7',
          '1': '#f7f7f7',
          '2': '#efefef',
          '3': '#e6e6e6',
          '4': '#dedede',
          '5': '#d3d3d3',
        },
        'gray': {
          DEFAULT: '#848484',
          '1': '#c8c8c8',
          '2': '#adadad',
          '3': '#848484',
          '4': '#535353',
          '5': '#3b3b3b',
        },
        'dark': {
          DEFAULT: '#0e0e0e',
          '1': '#323232',
          '2': '#262626',
          '3': '#1f1f1f',
          '4': '#181818',
          '5': '#0e0e0e',
        },
        'black': '#000000',
        'accent': {
          DEFAULT: '#E91E63',
          '1': '#ED437D',
          '2': '#EB3170',
          '3': '#E91E63',
          '4': '#DD1659',
          '5': '#CA1452',
        },
      },
      fontFamily: {
        'inter': ['"Inter"', 'Arial', 'sans-serif'],
        'roboto': ['"Roboto"', 'Arial', 'sans-serif'],
      },
      fontSize: {
        '1': "0.25rem",
        '2': "0.5rem",
        '3': "0.75rem",
        '3.25': "0.8125rem",
        '3.5': "0.875rem",
        '3.75': "0.9375rem",
        '4': "1rem",
      },
      boxShadow: {
        navbar: "0 1px 2px rgba(0,0,0,0.9),0 0px 2px rgba(0,0,0,0.9)"
      }
    },
  },
  variants: {
    extend: {},
  },
  plugins: [],
}
# Orderbook

An orderbook in rust that analyzes https://docs.tardis.dev/downloadable-csv-files#incremental_book_l2
data and uses multithreading to simulate a real-time orderbook


This data is just a static .csv, but maybe in the future we'll do some websockets stuff to listen for new orders off of Binance or something, idk.



each node in the RB tree will be keyed by the Limit price with each value being the doubly linked list of Orders.



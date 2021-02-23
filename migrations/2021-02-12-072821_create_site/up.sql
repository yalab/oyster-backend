CREATE TABLE sites (
  id SERIAL PRIMARY KEY,
  url VARCHAR NOT NULL,
  crawled_at timestamp
);

INSERT INTO sites (url) VALUES ('https://suumo.jp/jj/chintai/ichiran/FR301FC001/?ar=030&bs=040&ra=013&cb=0.0&ct=9999999&et=9999999&cn=9999999&mb=0&mt=9999999&shkr1=03&shkr2=03&shkr3=03&shkr4=03&fw2=&ek=001527320&rn=0015') 

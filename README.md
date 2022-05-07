## PostgreSQL Extension to generate various variant of Slugs from a string.
  

##  1. Installation

### 1.1. Installation on non docker environment

#### 1.1.1. Install rust through rustup.
  
  ```bash
curl https://sh.rustup.rs -sSf | sh
  ```
#### 1.1.2. Prepare your postgres installation
#### 1.1.3. Install pgx
  
  ```bash
cargo install cargo-pgx
  ```
#### 1.1.4. Initialize pgx for the postgres version you have already installed
    
- Note: Handle the number accordingly.
```bash
cargo pgx init --pg14 $(which pg_config)
```
#### 1.1.5. Install the extension
    
```bash
git clone https://github.com/spa5k/slugify-postgres \
&& cd slugify-postgres \
&& cargo pgx install
```

### 1.2. Installation on docker environment

  Check the included [Dockerfile](./docker/Dockerfile) for the installation template.

##  2. Functions available

### 2.1. Enable the extension
  
```sql
CREATE EXTENSION IF NOT EXISTS slugify;
```

###  2.2. Slug -


#### 2.2.1. Generate Slug

  

```sql

select slug('hello world');

-- hello-world

-----------------------------

select slug('Æúű--cool?');

-- aeuu-cool

-----------------------------

select slug('jaja---lol-méméméoo--a');

-- jaja-lol-mememeoo-a

-----------------------------
```
> It even supports unicode characters.
```sql
select slug('影師嗎');

-- ying-shi-ma

```

#### 2.2.2. Generate slug with random suffix.

> Note - rand by default adds a 5 character random suffix. and add it to parent string using the separator that is used, else the default separator if none is mentioned.

> These slugs are generated by appending a string generated through [nanoid](https://github.com/ai/nanoid)  with length of 5 as default, you can also change the length.


```sql

select slug_rand('Компьютер');

-- komp-iuter-lkai

-----------------------------

select slug_rand('Æúű--cool?');

-- aeuu-cool-3epv

-----------------------------

```

  

###  2.3. Slug with different separator -

  

#### 2.3.1. Generate a slug with different separator.

  

```sql

select slug_sep('hello world', '_');

-----------------------------

-- hello_world

select slug_sep('Of course can be paid','*');

-----------------------------

-- of*course*can*be*paid

```

  

#### 2.3.2. Generate a slug with different separator and random suffix.

  

```sql

select slug_rand_sep('heLLo WorlD','_');

-- hello_world_obc5

-----------------------------

select slug_rand_sep('Of course can be paid','*');

-- of*course*can*be*paid*3ibd

-----------------------------
```


###  2.4. Custom Length of random  suffix-

  

#### 2.4.1. Generate a slug with custom length of random suffix.

  

```sql

select slug_rand_c('the 101 dalmatians', 10);


-- the-101-dalmatians-mrjygfcnr

-----------------------------
```

  

#### 2.4.2. Generate a slug with custom length of random suffix and custom separator.

  

```sql

select slug_rand_sep_c('the 101 dalmatians', '_',10);

-- the_101_dalmatians_5en4hhnrt

-----------------------------
```

## 3. Add the trigger.

> Typically you will only want to create a slug when a record is created. If you update the title of something, you probably want to preserve the slug as it may well be part of your websites’ URL structure. You could update your slug each time, but you’d need an audit table to redirect people from the previous slug(s) to the new ones.

> Depending on your schema, you can have a function and trigger for each table that requires a slug, OR if you have consistent column names over tables you can create a generic function that faithfully assumes you’ve got a title and a slug column:


```sql
CREATE FUNCTION public.set_slug_from_title() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
  NEW.slug := slug(NEW.title);
  RETURN NEW;
END
$$;
```

> This function returns a trigger, whilst the slugify function returns text. The NEW keyword above is effectively referencing the table row we are updating.

>Note that the above function will happily generate duplicate slugs. You could append an ID, hashid or some other bit of random text to shoot for uniqueness.

> And finally, to add this trigger to your table(s)…

```sql
CREATE TRIGGER "slugify_the_name" BEFORE INSERT ON "table_name" FOR EACH ROW WHEN (NEW.title IS NOT NULL AND NEW.slug IS NULL)
EXECUTE PROCEDURE set_slug_from_title();
```

## 4. More usecases
> Please check the tests below for more possible usecases.

```rust
assert_eq!("hello-world", slug("hello world"));
assert_eq!("hello_world", slug_sep("hello world", "_"));
assert_eq!("helloworld", slug_sep("hello world", ""));
assert_eq!("hello%world", slug_sep("hello world", "%%"));
assert_eq!(slug("alice@bob.com"), "alice-bob-com");
assert_eq!(slug("alice@bob.com"), "alice-bob-com");
assert_eq!(slug("10 amazing secrets"), "10-amazing-secrets");
assert_eq!(slug("the 101 dalmatians"), "the-101-dalmatians");
assert_eq!(
    slug_rand("the 101 dalmatians").len(),
    "the-101-dalmatians".len() + 5
);
assert_eq!(
    slug_rand_c("the 101 dalmatians", 10).len(),
    "the-101-dalmatians".len() + 10
);
assert_eq!(
    slug_rand_sep_c("the 101 dalmatians", "_", 10).len(),
    "the-101-dalmatians".len() + 10
);
assert_eq!(slug("lucky number 7"), "lucky-number-7");
assert_eq!(
    slug("1000 reasons you are #1"),
    "1000-reasons-you-are-1"
);
assert_eq!(slug_sep("hello world", "."), "hello.world");
assert_eq!(slug_sep("hello world", "_"), "hello_world");
assert_eq!(
    slug_rand_sep("hello world-", "_").len(),
    "hello_world".len() + 5
);
assert_eq!(slug("影師嗎"), "ying-shi-ma");
assert_eq!(slug("Æúű--cool?"), "aeuu-cool");
assert_eq!(
    slug("Nín hǎo. Wǒ shì zhōng guó rén"),
    "nin-hao-wo-shi-zhong-guo-ren"
);
assert_eq!(slug("jaja---lol-méméméoo--a"), "jaja-lol-mememeoo-a");
assert_eq!(slug("Компьютер"), "komp-iuter");
assert_eq!(slug("Компьютер"), "komp-iuter");
assert_eq!(slug_sep("hello world", "-"), "hello-world");
assert_eq!(slug_sep("hello world", " "), "hello world");
assert_eq!(
    "hello-world".len() + 5,
    slug_rand("hello world").len()
);
assert_eq!(
    "hello_world".len() + 5,
    slug_rand_sep("hello world", "_").len()
);
```


## 5. Acknowledgements

### 5.1. [@kez](https://www.kdobson.net/2019/ultimate-postgresql-slug-function/)
### 5.2. [@slugify-rs](https://github.com/spa5k/slugify-rs)
### 5.3. [@pgx](https://github.com/tcdi/pgx)
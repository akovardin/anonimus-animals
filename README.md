# Анонимные животные

Знаете эти милые аватарки с животными в общих Google-документах? Они появляются у каждого, кто открывает файл по ссылке. Выбрать себе зверюшку заранее или как-то на неё повлиять нельзя - ваш аватар назначается случайно. Чтобы узнать, в кого вы "превратились", придется спросить у кого-нибудь из тех, кто в документе вместе с вами. Хочу реализовать похожую механику на kodikapusta.ru

## Запуск

```
docker run -d -v /some/data:/app/data -p 8888:8000 registry.gitflic.ru/project/kovardin/anonimus-animals/anonimus-animals:latest
```

Больше подробностей в статье [kodikapusta.ru/articles/rust-anonimus-animals](https://kodikapusta.ru/articles/rust-anonimus-animals)
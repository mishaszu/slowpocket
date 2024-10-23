INSERT INTO
  users (id, email, hash, is_admin)
VALUES
  (
    'a74f9b43-8a49-4d97-8270-9879d37c600d',
    'test@myemail.com',
    -- dev_only_pass
    '$argon2id$v=19$m=19456,t=2,p=1$l9VfAtWMe+bWqP81cgsDuQ$Z+ExthpqUCPuHSwxtHI1RP17OyVGo1/bapupD+cJYzw',
    true
  );

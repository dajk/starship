# CÂU HỎI THƯỜNG GẶP

## Cấu hình sử dụng trong demo GIF là gì?

- **Terminal Emulator**: [iTerm2](https://iterm2.com/)
  - **Theme**: Minimal
  - **Color Scheme**: [Snazzy](https://github.com/sindresorhus/iterm2-snazzy)
  - **Font**: [FiraCode Nerd Font](https://www.nerdfonts.com/font-downloads)
- **Shell**: [Fish Shell](https://fishshell.com/)
  - **Cấu hình**: [matchai's Dotfiles](https://github.com/matchai/dotfiles/blob/b6c6a701d0af8d145a8370288c00bb9f0648b5c2/.config/fish/config.fish)
  - **Prompt**: [Starship](https://starship.rs/)

## Làm thế nào tôi có được hoàn thiện câu lệnh như được hiển thị trong demo GIF?

Hỗ trợ hoàn thiện, hoặc tự động hoàn thiện, được cung cấp bởi shell của bạn lựa chọn. Trong trường hợp của demo, demo đã thực hiện với [Fish Shell](https://fishshell.com/), thứ mặc định cung cấp những completion. Nếu bạn sử dụng Z Shell (zsh), tôi muốn gợi ý xem qua [zsh-autosuggestions](https://github.com/zsh-users/zsh-autosuggestions).

## Có phải level trên cùng `format` và `<module>.disable` thực hiện cùng một thứ giống nhau?

Vâng, chúng có thể cùng được sử dụng để vô hiệu hoá các module trong prompt. Nếu tất cả bạn những việc bạn làm để vô hiệu hoá các module, `<module>.disabled` là các được ưa thích hơn để thực hiện với những lí do sau:

- Các module vô hiệu là ro ràng hơn sót chúng từ level trên cùng `format`
- Những module mới tạo sẽ được thêm bào prompt khi Starship được cập nhật

## Các tài liệu nói Starship là cross-shell. Tại sao shell ưa thích của tôi không được hỗ trợ?

Cách Starship được xây dựng, nó nên khả thi để thêm hỗ trợ với bất kì shell ảo nào. Starship nhị phân là phi trạng thái và bất khả tri của shell, vì vậy, miễn là shell của bạn hỗ trợ tùy chỉnh nhanh chóng và mở rộng shell, thì Starship có thể được sử dụng.

Đây là một ví dụ nhỏ để Starship là việc với bash:

```sh
# Lấy mã trạng thái từ câu lệnh cuối đã được thực thi
STATUS=$?

# Lấy số lượng job đang chạy.
NUM_JOBS=$(jobs -p | wc -l)

# Thiết lập prompt thành đầu ra của `starship prompt`
PS1="$(starship prompt --status=$STATUS --jobs=$NUM_JOBS)"
```

[Bản cài đặt Bash](https://github.com/starship/starship/blob/master/src/init/starship.bash) xây dựng bên trong Starship phực tạp hơn một chút để cho phép thực hiện các tính năng nâng cao hơn một chứt như [Command Duration module](https://starship.rs/config/#Command-Duration) và chắc chắn rằng Starship là tương thích với cấu hình Bash đã cài đặt trước đó.

Với một danh sách tất cả các cờ đã được chấp nhận bởi `starship prompt`, sử dụng lệnh sau:

```sh
starship prompt --help
```

Prompt sẽ sử dụng nhiều ngữ cảnh được cung câos, nhưng không có cờ nào là "yêu cầu".

## Làm thế nào để tôi chạy Starship trên các bản phân phối Linux với các phiên bản cũ của glibc?

Nếu bạn nhận được một lỗi giống như "_version 'GLIBC_2.18' not found (required by starship)_" khi sử dụng prebuilt binary (ví dụ trên CentOS 6 hoặc 7), bạn có thể sử dụng tập tin đã được dịch với `musl` thay vì `glibc`:

```sh
curl -fsSL https://starship.rs/install.sh | bash -s -- --platform unknown-linux-musl
```

## Tại sao tôi không tình thấy một kí hiệu glyph trong prompt?

Đa số nguyên nhân phổ biến của việc này là cấu hình hệ thống sai. Một số bản phân phối Linux đặc biệt không có hỗ trợ phông chữ ngay lập tức. Bạn cần chắc chắn rằng:

- Mã ngôn ngữ của bạn được thiết lập là một giá trị UTF-8, giống như `de_DE.UTF-8` or `ja_JP.UTF-8`. Nếu `LO_ALL` không phải là một giá trị UTF-8, [ bạn sẽ cần thay đổi nó](https://www.tecmint.com/set-system-locales-in-linux/).
- Bạn đã cài đặt phông chữ biểu tượng cảm xúc. Đa số hệ thống đi kèm với một phông biểu tượng cảm xúc mặc định, nhưng một vài (đáng chú ý là Arch Linux) thì không. Bạn có thể thường cài đặt thông qua một trình quản lí gói hệ thống của bạn--[noto emoji](https://www.google.com/get/noto/help/emoji/) là một lựa chọn phổ biến.
- Bạn đang sử dụng một [Nerd Font](https://www.nerdfonts.com/).

Để kiểm tra hệ thống của bạn, chạy các câu lệnh sau trong một terminal:

```sh
echo -e "\xf0\x9f\x90\x8d"
echo -e "\xee\x82\xa0"
```

Dòng đầu tiên nên sinh ra một [snake emoji](https://emojipedia.org/snake/), trong khi dòng thứ hai nên sinh ra một [powerline branch symbol (e0a0)](https://github.com/ryanoasis/powerline-extra-symbols#glyphs).

Nếu một trong hai biểu tượng không hiển thị chính xác, hệ thống của bạn vẫn bị cấu hình sai. Thật không may, việc lấy đúng cấu hình phông chữ đôi khi rất khó. Những người dùng trên Discord có thể giúp đỡ. Nếu cả hai kí hiệu hiển thị đúng, nhưng bạn vẫn không thấy chúng trong starship, [nộp một báo cáo lỗi!](https://github.com/starship/starship/issues/new/choose)

## Làm cách nào để tôi gỡ cài đặt Starship?

Starship thì dễ dàng gỡ cài đặt như cài đặt ngay từ đầu.

1. Gỡ mọi tập tin trong cấu hình shell của bạn (e.g. `~/.bashrc`) đã sử dụng để khởi tạo Starship.
1. Xoá tập tin Starship nhị phân.

Nếu Starship đã được cài đặt bằng việc sử dụng một trình quản lí gói, vui lòng tham khảo tài liệu của chúng để gỡ cài đặt.

Nếu Starship đã được càu đặt bằng việc sử dụng `curl | bash`, theo câu lệnh sau để xoá tập tin nhị phân:

```sh
# Xác định vị trí và xóa tập tin nhị phân của starship
rm "$(which starship)"
```
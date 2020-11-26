FROM archlinux/base

RUN pacman -Sy base-devel libpng yarn --noconfirm

WORKDIR client
RUN yarn install
CMD ["yarn","serve"]

NAME := scop
SRC_DIR := resource

release:
	@cd $(SRC_DIR) && cargo build --release
	@cp $(SRC_DIR)/target/release/$(SRC_DIR) ./$(NAME)

debug:
	@cd $(SRC_DIR) && cargo build
	@cp $(SRC_DIR)/target/debug/$(SRC_DIR) ./$(NAME)

test:
	@cd $(SRC_DIR) && cargo test

lint:
	@cd $(SRC_DIR) && cargo clippy --all-targets --all-features

fmt-check:
	@cd $(SRC_DIR) && cargo fmt --all --check

fmt:
	@cd $(SRC_DIR) && cargo fmt --all

$(NAME): release

all: release

clean:
	@cd $(SRC_DIR) && cargo clean

fclean: clean
	@rm -f ./$(NAME)

re: fclean all

.PHONY: all clean fclean re release debug test lint fmt-check fmt
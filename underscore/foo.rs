Program {
    structs: [
        Spanned {
            span: Span {
                start: Position {
                    line: 1,
                    column: 1,
                    absolute: 0
                },
                end: Position {
                    line: 4,
                    column: 1,
                    absolute: 41
                }
            },
            value: Struct {
                span: Span {
                    start: Position {
                        line: 1,
                        column: 1,
                        absolute: 0
                    },
                    end: Position {
                        line: 1,
                        column: 7,
                        absolute: 6
                    }
                },
                name: Spanned {
                    span: Span {
                        start: Position {
                            line: 1,
                            column: 8,
                            absolute: 7
                        },
                        end: Position {
                            line: 1,
                            column: 12,
                            absolute: 11
                        }
                    },
                    value: ItemName {
                        name: Spanned {
                            span: Span {
                                start: Position {
                                    line: 1,
                                    column: 8,
                                    absolute: 7
                                },
                                end: Position {
                                    line: 1,
                                    column: 12,
                                    absolute: 11
                                }
                            },
                            value: Symbol(
                                0
                            )
                        },
                        type_params: []
                    }
                },
                fields: Spanned {
                    span: Span {
                        start: Position {
                            line: 1,
                            column: 13,
                            absolute: 12
                        },
                        end: Position {
                            line: 4,
                            column: 1,
                            absolute: 41
                        }
                    },
                    value: [
                        Spanned {
                            span: Span {
                                start: Position {
                                    line: 2,
                                    column: 5,
                                    absolute: 18
                                },
                                end: Position {
                                    line: 2,
                                    column: 13,
                                    absolute: 26
                                }
                            },
                            value: Field {
                                name: Spanned {
                                    span: Span {
                                        start: Position {
                                            line: 2,
                                            column: 5,
                                            absolute: 18
                                        },
                                        end: Position {
                                            line: 2,
                                            column: 9,
                                            absolute: 22
                                        }
                                    },
                                    value: Symbol(
                                        1
                                    )
                                },
                                ty: Spanned {
                                    span: Span {
                                        start: Position {
                                            line: 2,
                                            column: 10,
                                            absolute: 23
                                        },
                                        end: Position {
                                            line: 2,
                                            column: 13,
                                            absolute: 26
                                        }
                                    },
                                    value: I32
                                }
                            }
                        },
                        Spanned {
                            span: Span {
                                start: Position {
                                    line: 3,
                                    column: 5,
                                    absolute: 32
                                },
                                end: Position {
                                    line: 3,
                                    column: 13,
                                    absolute: 40
                                }
                            },
                            value: Field {
                                name: Spanned {
                                    span: Span {
                                        start: Position {
                                            line: 3,
                                            column: 5,
                                            absolute: 32
                                        },
                                        end: Position {
                                            line: 3,
                                            column: 9,
                                            absolute: 36
                                        }
                                    },
                                    value: Symbol(
                                        2
                                    )
                                },
                                ty: Spanned {
                                    span: Span {
                                        start: Position {
                                            line: 3,
                                            column: 10,
                                            absolute: 37
                                        },
                                        end: Position {
                                            line: 3,
                                            column: 13,
                                            absolute: 40
                                        }
                                    },
                                    value: Nil
                                }
                            }
                        }
                    ]
                }
            }
        }
    ],
    functions: [
        Spanned {
            span: Span {
                start: Position {
                    line: 6,
                    column: 1,
                    absolute: 44
                },
                end: Position {
                    line: 9,
                    column: 1,
                    absolute: 88
                }
            },
            value: Function {
                span: Span {
                    start: Position {
                        line: 6,
                        column: 1,
                        absolute: 44
                    },
                    end: Position {
                        line: 9,
                        column: 1,
                        absolute: 88
                    }
                },
                name: Spanned {
                    span: Span {
                        start: Position {
                            line: 6,
                            column: 4,
                            absolute: 47
                        },
                        end: Position {
                            line: 6,
                            column: 7,
                            absolute: 50
                        }
                    },
                    value: ItemName {
                        name: Spanned {
                            span: Span {
                                start: Position {
                                    line: 6,
                                    column: 4,
                                    absolute: 47
                                },
                                end: Position {
                                    line: 6,
                                    column: 7,
                                    absolute: 50
                                }
                            },
                            value: Symbol(
                                3
                            )
                        },
                        type_params: []
                    }
                },
                params: Spanned {
                    span: Span {
                        start: Position {
                            line: 6,
                            column: 7,
                            absolute: 50
                        },
                        end: Position {
                            line: 6,
                            column: 8,
                            absolute: 51
                        }
                    },
                    value: []
                },
                returns: None,
                body: Spanned {
                    span: Span {
                        start: Position {
                            line: 6,
                            column: 10,
                            absolute: 53
                        },
                        end: Position {
                            line: 9,
                            column: 1,
                            absolute: 88
                        }
                    },
                    value: Block(
                        [
                            Spanned {
                                span: Span {
                                    start: Position {
                                        line: 7,
                                        column: 5,
                                        absolute: 59
                                    },
                                    end: Position {
                                        line: 7,
                                        column: 17,
                                        absolute: 71
                                    }
                                },
                                value: Let {
                                    escapes: false,
                                    ident: Spanned {
                                        span: Span {
                                            start: Position {
                                                line: 7,
                                                column: 9,
                                                absolute: 63
                                            },
                                            end: Position {
                                                line: 7,
                                                column: 10,
                                                absolute: 64
                                            }
                                        },
                                        value: Symbol(
                                            4
                                        )
                                    },
                                    ty: None,
                                    expr: Some(
                                        Spanned {
                                            span: Span {
                                                start: Position {
                                                    line: 7,
                                                    column: 13,
                                                    absolute: 67
                                                },
                                                end: Position {
                                                    line: 7,
                                                    column: 17,
                                                    absolute: 71
                                                }
                                            },
                                            value: Literal(
                                                True(
                                                    true
                                                )
                                            )
                                        }
                                    )
                                }
                            },
                            Spanned {
                                span: Span {
                                    start: Position {
                                        line: 8,
                                        column: 14,
                                        absolute: 86
                                    },
                                    end: Position {
                                        line: 8,
                                        column: 14,
                                        absolute: 86
                                    }
                                },
                                value: Expr(
                                    Spanned {
                                        span: Span {
                                            start: Position {
                                                line: 8,
                                                column: 5,
                                                absolute: 77
                                            },
                                            end: Position {
                                                line: 8,
                                                column: 13,
                                                absolute: 85
                                            }
                                        },
                                        value: Assign {
                                            name: Spanned {
                                                span: Span {
                                                    start: Position {
                                                        line: 8,
                                                        column: 5,
                                                        absolute: 77
                                                    },
                                                    end: Position {
                                                        line: 8,
                                                        column: 6,
                                                        absolute: 78
                                                    }
                                                },
                                                value: Simple(
                                                    Spanned {
                                                        span: Span {
                                                            start: Position {
                                                                line: 8,
                                                                column: 5,
                                                                absolute: 77
                                                            },
                                                            end: Position {
                                                                line: 8,
                                                                column: 6,
                                                                absolute: 78
                                                            }
                                                        },
                                                        value: Symbol(
                                                            4
                                                        )
                                                    }
                                                )
                                            },
                                            value: Spanned {
                                                span: Span {
                                                    start: Position {
                                                        line: 8,
                                                        column: 9,
                                                        absolute: 81
                                                    },
                                                    end: Position {
                                                        line: 8,
                                                        column: 13,
                                                        absolute: 85
                                                    }
                                                },
                                                value: Literal(
                                                    True(
                                                        true
                                                    )
                                                )
                                            }
                                        }
                                    }
                                )
                            }
                        ]
                    )
                },
                linkage: Normal
            }
        },
        Spanned {
            span: Span {
                start: Position {
                    line: 12,
                    column: 1,
                    absolute: 92
                },
                end: Position {
                    line: 20,
                    column: 1,
                    absolute: 212
                }
            },
            value: Function {
                span: Span {
                    start: Position {
                        line: 12,
                        column: 1,
                        absolute: 92
                    },
                    end: Position {
                        line: 20,
                        column: 1,
                        absolute: 212
                    }
                },
                name: Spanned {
                    span: Span {
                        start: Position {
                            line: 12,
                            column: 4,
                            absolute: 95
                        },
                        end: Position {
                            line: 12,
                            column: 8,
                            absolute: 99
                        }
                    },
                    value: ItemName {
                        name: Spanned {
                            span: Span {
                                start: Position {
                                    line: 12,
                                    column: 4,
                                    absolute: 95
                                },
                                end: Position {
                                    line: 12,
                                    column: 8,
                                    absolute: 99
                                }
                            },
                            value: Symbol(
                                5
                            )
                        },
                        type_params: []
                    }
                },
                params: Spanned {
                    span: Span {
                        start: Position {
                            line: 12,
                            column: 8,
                            absolute: 99
                        },
                        end: Position {
                            line: 12,
                            column: 9,
                            absolute: 100
                        }
                    },
                    value: []
                },
                returns: None,
                body: Spanned {
                    span: Span {
                        start: Position {
                            line: 12,
                            column: 11,
                            absolute: 102
                        },
                        end: Position {
                            line: 20,
                            column: 1,
                            absolute: 212
                        }
                    },
                    value: Block(
                        [
                            Spanned {
                                span: Span {
                                    start: Position {
                                        line: 13,
                                        column: 5,
                                        absolute: 108
                                    },
                                    end: Position {
                                        line: 13,
                                        column: 15,
                                        absolute: 118
                                    }
                                },
                                value: Let {
                                    escapes: false,
                                    ident: Spanned {
                                        span: Span {
                                            start: Position {
                                                line: 13,
                                                column: 9,
                                                absolute: 112
                                            },
                                            end: Position {
                                                line: 13,
                                                column: 10,
                                                absolute: 113
                                            }
                                        },
                                        value: Symbol(
                                            4
                                        )
                                    },
                                    ty: None,
                                    expr: Some(
                                        Spanned {
                                            span: Span {
                                                start: Position {
                                                    line: 13,
                                                    column: 13,
                                                    absolute: 116
                                                },
                                                end: Position {
                                                    line: 13,
                                                    column: 15,
                                                    absolute: 118
                                                }
                                            },
                                            value: Literal(
                                                Number(
                                                    Number {
                                                        value: 10,
                                                        ty: None
                                                    }
                                                )
                                            )
                                        }
                                    )
                                }
                            },
                            Spanned {
                                span: Span {
                                    start: Position {
                                        line: 14,
                                        column: 5,
                                        absolute: 124
                                    },
                                    end: Position {
                                        line: 18,
                                        column: 5,
                                        absolute: 192
                                    }
                                },
                                value: Block(
                                    [
                                        Spanned {
                                            span: Span {
                                                start: Position {
                                                    line: 15,
                                                    column: 9,
                                                    absolute: 134
                                                },
                                                end: Position {
                                                    line: 15,
                                                    column: 25,
                                                    absolute: 150
                                                }
                                            },
                                            value: Let {
                                                escapes: false,
                                                ident: Spanned {
                                                    span: Span {
                                                        start: Position {
                                                            line: 15,
                                                            column: 13,
                                                            absolute: 138
                                                        },
                                                        end: Position {
                                                            line: 15,
                                                            column: 14,
                                                            absolute: 139
                                                        }
                                                    },
                                                    value: Symbol(
                                                        6
                                                    )
                                                },
                                                ty: None,
                                                expr: Some(
                                                    Spanned {
                                                        span: Span {
                                                            start: Position {
                                                                line: 15,
                                                                column: 16,
                                                                absolute: 141
                                                            },
                                                            end: Position {
                                                                line: 15,
                                                                column: 25,
                                                                absolute: 150
                                                            }
                                                        },
                                                        value: Literal(
                                                            Str(
                                                                "aaaaa\\0"
                                                            )
                                                        )
                                                    }
                                                )
                                            }
                                        },
                                        Spanned {
                                            span: Span {
                                                start: Position {
                                                    line: 16,
                                                    column: 16,
                                                    absolute: 167
                                                },
                                                end: Position {
                                                    line: 16,
                                                    column: 16,
                                                    absolute: 167
                                                }
                                            },
                                            value: Expr(
                                                Spanned {
                                                    span: Span {
                                                        start: Position {
                                                            line: 16,
                                                            column: 10,
                                                            absolute: 161
                                                        },
                                                        end: Position {
                                                            line: 16,
                                                            column: 16,
                                                            absolute: 167
                                                        }
                                                    },
                                                    value: Assign {
                                                        name: Spanned {
                                                            span: Span {
                                                                start: Position {
                                                                    line: 16,
                                                                    column: 10,
                                                                    absolute: 161
                                                                },
                                                                end: Position {
                                                                    line: 16,
                                                                    column: 11,
                                                                    absolute: 162
                                                                }
                                                            },
                                                            value: Simple(
                                                                Spanned {
                                                                    span: Span {
                                                                        start: Position {
                                                                            line: 16,
                                                                            column: 10,
                                                                            absolute: 161
                                                                        },
                                                                        end: Position {
                                                                            line: 16,
                                                                            column: 11,
                                                                            absolute: 162
                                                                        }
                                                                    },
                                                                    value: Symbol(
                                                                        4
                                                                    )
                                                                }
                                                            )
                                                        },
                                                        value: Spanned {
                                                            span: Span {
                                                                start: Position {
                                                                    line: 16,
                                                                    column: 14,
                                                                    absolute: 165
                                                                },
                                                                end: Position {
                                                                    line: 16,
                                                                    column: 16,
                                                                    absolute: 167
                                                                }
                                                            },
                                                            value: Literal(
                                                                Number(
                                                                    Number {
                                                                        value: 32,
                                                                        ty: None
                                                                    }
                                                                )
                                                            )
                                                        }
                                                    }
                                                }
                                            )
                                        },
                                        Spanned {
                                            span: Span {
                                                start: Position {
                                                    line: 17,
                                                    column: 18,
                                                    absolute: 186
                                                },
                                                end: Position {
                                                    line: 17,
                                                    column: 18,
                                                    absolute: 186
                                                }
                                            },
                                            value: Expr(
                                                Spanned {
                                                    span: Span {
                                                        start: Position {
                                                            line: 17,
                                                            column: 10,
                                                            absolute: 178
                                                        },
                                                        end: Position {
                                                            line: 17,
                                                            column: 18,
                                                            absolute: 186
                                                        }
                                                    },
                                                    value: Cast {
                                                        from: Spanned {
                                                            span: Span {
                                                                start: Position {
                                                                    line: 17,
                                                                    column: 10,
                                                                    absolute: 178
                                                                },
                                                                end: Position {
                                                                    line: 17,
                                                                    column: 11,
                                                                    absolute: 179
                                                                }
                                                            },
                                                            value: Var(
                                                                Spanned {
                                                                    span: Span {
                                                                        start: Position {
                                                                            line: 17,
                                                                            column: 10,
                                                                            absolute: 178
                                                                        },
                                                                        end: Position {
                                                                            line: 17,
                                                                            column: 11,
                                                                            absolute: 179
                                                                        }
                                                                    },
                                                                    value: Simple(
                                                                        Spanned {
                                                                            span: Span {
                                                                                start: Position {
                                                                                    line: 17,
                                                                                    column: 10,
                                                                                    absolute: 178
                                                                                },
                                                                                end: Position {
                                                                                    line: 17,
                                                                                    column: 11,
                                                                                    absolute: 179
                                                                                }
                                                                            },
                                                                            value: Symbol(
                                                                                4
                                                                            )
                                                                        }
                                                                    )
                                                                }
                                                            )
                                                        },
                                                        to: Spanned {
                                                            span: Span {
                                                                start: Position {
                                                                    line: 17,
                                                                    column: 15,
                                                                    absolute: 183
                                                                },
                                                                end: Position {
                                                                    line: 17,
                                                                    column: 18,
                                                                    absolute: 186
                                                                }
                                                            },
                                                            value: I32
                                                        }
                                                    }
                                                }
                                            )
                                        }
                                    ]
                                )
                            },
                            Spanned {
                                span: Span {
                                    start: Position {
                                        line: 18,
                                        column: 7,
                                        absolute: 194
                                    },
                                    end: Position {
                                        line: 18,
                                        column: 8,
                                        absolute: 195
                                    }
                                },
                                value: Block(
                                    []
                                )
                            },
                            Spanned {
                                span: Span {
                                    start: Position {
                                        line: 19,
                                        column: 14,
                                        absolute: 210
                                    },
                                    end: Position {
                                        line: 19,
                                        column: 14,
                                        absolute: 210
                                    }
                                },
                                value: Expr(
                                    Spanned {
                                        span: Span {
                                            start: Position {
                                                line: 19,
                                                column: 5,
                                                absolute: 201
                                            },
                                            end: Position {
                                                line: 19,
                                                column: 13,
                                                absolute: 209
                                            }
                                        },
                                        value: Assign {
                                            name: Spanned {
                                                span: Span {
                                                    start: Position {
                                                        line: 19,
                                                        column: 5,
                                                        absolute: 201
                                                    },
                                                    end: Position {
                                                        line: 19,
                                                        column: 6,
                                                        absolute: 202
                                                    }
                                                },
                                                value: Simple(
                                                    Spanned {
                                                        span: Span {
                                                            start: Position {
                                                                line: 19,
                                                                column: 5,
                                                                absolute: 201
                                                            },
                                                            end: Position {
                                                                line: 19,
                                                                column: 6,
                                                                absolute: 202
                                                            }
                                                        },
                                                        value: Symbol(
                                                            4
                                                        )
                                                    }
                                                )
                                            },
                                            value: Spanned {
                                                span: Span {
                                                    start: Position {
                                                        line: 19,
                                                        column: 9,
                                                        absolute: 205
                                                    },
                                                    end: Position {
                                                        line: 19,
                                                        column: 13,
                                                        absolute: 209
                                                    }
                                                },
                                                value: Literal(
                                                    True(
                                                        true
                                                    )
                                                )
                                            }
                                        }
                                    }
                                )
                            }
                        ]
                    )
                },
                linkage: Normal
            }
        }
    ],
    type_alias: []
}
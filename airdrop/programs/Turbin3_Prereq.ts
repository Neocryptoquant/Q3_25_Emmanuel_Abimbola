export type Turbin3Prereq = {
    "version": "0.1.0",
    "name": "q3_pre_reqs_rs",
    "instructions": [
        {
            "name": "initialize",
            "accounts": [
                { "name": "user", "isMut": true, "isSigner": true },
                { "name": "account", "isMut": true },
                { "name": "systemProgram", "isMut": false }
            ],
            "args": [
                { "name": "github", "type": "string" }
            ]
        },
        {
            "name": "submitTs",
            "accounts": [
                { "name": "user", "isMut": true, "isSigner": true },
                { "name": "account", "isMut": true },
                { "name": "mint", "isMut": true, "isSigner": true },
                { "name": "collection", "isMut": true },
                { "name": "authority", "isMut": false },
                { "name": "mplCoreProgram", "isMut": false },
                { "name": "systemProgram", "isMut": false }
            ],
            "args": []
        }
    ],
    "accounts": [
        {
            "name": "applicationAccount",
            "type": {
                "kind": "struct",
                "fields": [
                    { "name": "user", "type": "publicKey" },
                    { "name": "bump", "type": "u8" },
                    { "name": "preReqTs", "type": "bool" },
                    { "name": "preReqRs", "type": "bool" },
                    { "name": "github", "type": "string" }
                ]
            }
        }
    ],
    "errors": [
        {
            "code": 6000,
            "name": "PreReqTsNotCompleted",
            "msg": "TS submission not completed."
        },
        {
            "code": 6001,
            "name": "PreReqTsAlreadyCompleted",
            "msg": "TS submission already completed."
        },
        {
            "code": 6002,
            "name": "PreReqRsAlreadyCompleted",
            "msg": "Rust submission already completed."
        },
        {
            "code": 6003,
            "name": "PreReqRsNotInTimeWindow",
            "msg": "Submission not allowed."
        }
    ]
};

export const IDL: Turbin3Prereq = {
    "version": "0.1.0",
    "name": "q3_pre_reqs_rs",
    "instructions": [
        {
            "name": "initialize",
            "accounts": [
                { "name": "user", "isMut": true, "isSigner": true },
                { "name": "account", "isMut": true },
                { "name": "systemProgram", "isMut": false }
            ],
            "args": [
                { "name": "github", "type": "string" }
            ]
        },
        {
            "name": "submitTs",
            "accounts": [
                { "name": "user", "isMut": true, "isSigner": true },
                { "name": "account", "isMut": true },
                { "name": "mint", "isMut": true, "isSigner": true },
                { "name": "collection", "isMut": true },
                { "name": "authority", "isMut": false },
                { "name": "mplCoreProgram", "isMut": false },
                { "name": "systemProgram", "isMut": false }
            ],
            "args": []
        }
    ],
    "accounts": [
        {
            "name": "applicationAccount",
            "type": {
                "kind": "struct",
                "fields": [
                    { "name": "user", "type": "publicKey" },
                    { "name": "bump", "type": "u8" },
                    { "name": "preReqTs", "type": "bool" },
                    { "name": "preReqRs", "type": "bool" },
                    { "name": "github", "type": "string" }
                ]
            }
        }
    ],
    "errors": [
        {
            "code": 6000,
            "name": "PreReqTsNotCompleted",
            "msg": "TS submission not completed."
        },
        {
            "code": 6001,
            "name": "PreReqTsAlreadyCompleted",
            "msg": "TS submission already completed."
        },
        {
            "code": 6002,
            "name": "PreReqRsAlreadyCompleted",
            "msg": "Rust submission already completed."
        },
        {
            "code": 6003,
            "name": "PreReqRsNotInTimeWindow",
            "msg": "Submission not allowed."
        }
    ]
};
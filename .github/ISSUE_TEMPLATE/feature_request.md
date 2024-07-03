---
name: Feature request
about: Suggest a new feature to Menix
title: "[Feature]"
labels: enhancement
assignees: ''
body:
    - type: markdown
      attributes:
        value: >
            **NOTE**: Please only use this form for submitting _well-formed_ proposals
            to extend or enhance Menix in some form. If you are trying to solve a problem,
            or need help bringing your idea into a actionable form, please use our [discussion section](https://github.com/menix-os/kernel/discussions/).
    - type: input
      attributes:
        label: Menix Version
        description: What version of Menix are you working with?
        placeholder: master
      validations:
            required: true
    - type: dropdown
      attributes:
        label: Feature type
        options:
            - New CPU architecture support
            - Expanding driver support
            - File System support
            - Change existing functionality
            - Change to the build system
            - Other
        validations:
            required: true
    - type: dropdown
      attributes:
        label: Architecture
        description: >
            What architecture will this proposal affect.
            (If no specific or all are affected, select 'None'.)
        options:
            - x86_64
            - aarch64/arm64
            - riscv64
            - None
            - New Architecture
    - type: dropdown
      attributes:
        label: Affected Area
        description: >
            What sub system of Menix does you proposal affect?
        options:
            - Boot
            - Drivers
            - File System
            - Main
            - Build System
            - Other
        validations:
            required: true
    - type: textarea
      attributes:
        description: >
            Describe your proposal in detail. Include any specific requirements
            for your plan to work as well as the supposed benefit your feature will bring.

            Also include a step by step actionplan you have drawn up to achieve this goal.
            Feature request with unclear or missing action plans cannot be acted upon.
        validations:
            required: true
---

; Fill in these cards as the deal goes on.
(define-cards flop "? ? ?")
(define-cards turn "?")
(define-cards river "?")
(define-cards community "$flop $turn $river")

; Fill in your cards here.
(plot-cards self "As Th $community")

; This line does not need changes.
(plot-cards opponents "? ? $community")


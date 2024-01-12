(define-cards flop "? ? ?")
(define-cards turn "?")
(define-cards river "?")
(define-cards community "$flop $turn $river")

(plot-cards self "As Th $community")
(plot-cards opponents "? ? $community")


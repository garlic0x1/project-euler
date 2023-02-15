(define (pe-31)
  (define coins '(1 2 5 10 20 50 100 200))
  (define (count-coins amount kinds)
    (cond ((= amount 0) 1)
          ((or (> 0 kinds) (> 0 amount)) 0)
          (else (+ (count-coins amount (dec kinds))
                   (count-coins (- amount (list-ref coins kinds)) kinds)))))
  (count-coins 200 (dec (length coins))))

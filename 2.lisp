(defmacro do-fibs (var limit &rest body)
  `(do ((i 0 (+ 1 i))
        (cur 0 next)
        (next 1 (+ cur next)))
       ((>= cur ,limit))
        (let ((,var cur))
          ,@body)))


(defun sum-evens (limit)
  (let ((sum 0))
    (do-fibs f limit
      (if (= 0 (mod f 2))
        (incf sum f))) sum ))

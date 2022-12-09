(defun factorial (num)
  (if (> num 1) (* num (factorial (- num 1))) 1))

(defun digits (num &optional (radix 10))
  (let ((ret))
    (loop while (< 0 num) do
      (let ((values (multiple-value-list (floor num radix))))
        (setq num (car values))
        (push (car (cdr values)) ret)))
    ret ))

(defun next-chain (num)
  (let ((sum 0))
    (dolist (dig (digits num))
      (incf sum (factorial dig)))
    sum ))

(defun contains (list num)
  (dolist (x list)
    (if (= x num)
        (return-from contains t)))
  nil )

(defun n-non-repeating (start target)
  (let ((list))
    (dotimes (x (+ 1 target))
        (if (contains list start)
          (if (= x target)
              (return-from n-non-repeating t)
              (return-from n-non-repeating nil))
          (progn
            (push start list)
            (setq start (next-chain start)))))
    nil))

(defun count-chains (under target)
  (let ((count 0))
  (dotimes (start under)
    (if (n-non-repeating start target)
        (incf count)))
    count))

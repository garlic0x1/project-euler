(setf chains (make-array '(10000000) :initial-element nil))
(setf (aref chains 1) 1)
(setf (aref chains 89) 89)

(defun safe-set (array ind value)
  (if (< ind (length array))
    (setf (aref array ind) value)))

(defun safe-ref (array ind)
  (if (< ind (length array))
      (aref array ind)))

(defun digits (num &optional (radix 10))
  (let ((ret))
    (loop while (< 0 num) do
      (let ((values (multiple-value-list (floor num radix)))) 
        (setq num (car values))
        (push (car (cdr values)) ret)))
    ret ))

(defun square-sum (num)
  (let ((sum 0))
    (dolist (dig (digits num))
      (incf sum (* dig dig)))
    sum ))

(defun switch-chain (inds dest)
  (dolist (i inds)
    (safe-set chains i dest)))

(defun travel (num)
  (let ((visited) (done nil))
    (loop do
      (let ((cell (or (safe-ref chains num) (square-sum num))))
        (push num visited)
        (cond
          ((and (/= cell 1) (/= cell 89))
            (setq num cell))
          ((= cell 1)
           (progn
             (switch-chain visited 1)
             (setq done t)))
          ((= cell 89)
           (progn
             (switch-chain visited 89)
             (setq done t)))))
      while (not done))))

(defun untangle ()
  (dotimes (ind (+ 1 (length chains)))    
    (travel (+ 1 ind)))
  (let ((sum 0))
    (dotimes (ind (length chains))
      (if (equal 89 (aref chains ind))
          (incf sum)))
    (print sum)))

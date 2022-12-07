(defvar chain (make-array '(1000000) :initial-element nil))

(defun next-collatz (num)
  (if (> 1 num) (return-from next-collatz 1))
  (if (= 0 (mod num 2)) (/ num 2) (+ 1 (* 3 num))))

(defun steps-to-one (num)
  (let ((sum 1))
    (progn
      (loop
        while (< 1 num)
        do (setf num (next-collatz num))
        do (if (< num 1000000)
          (if (aref chain num)
            (return-from steps-to-one
              (+ sum (aref chain num)))))
        do (incf sum))) sum ))

(defun solve2 ()
  (let ((res 0) (max 1))
    (dotimes (i 1000000)
      (let ((steps (steps-to-one i)))
        (setf (aref chain i) steps)
        (when (> steps max)
          (setf max steps)
          (setf res i)))) res ))

(defun solve ()
  (let ((res 0) (max 1))
    (progn 
      (loop for i from 1 to 999999
        do (let ((steps (steps-to-one i)))
          (progn
            (setf (aref chain i) steps)
            (if (> steps max)
              (progn
                (setf max steps)
                (setf res i))))))
      (return-from solve res))))

(time (print (solve)))

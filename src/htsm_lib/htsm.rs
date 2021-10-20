use ndarray;

#[derive(Debug)]
pub struct Htsm {
    mat: ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 2]>>,
}

impl Htsm {
    pub fn new(mat: ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 2]>>) -> Self {
        if mat.shape() == [4, 4] {
            return Htsm{mat};
        } else {
            panic!("Need a 4x4 matrix");
        }
    }

    pub fn transform(&self, mat_to_transform: &Htsm) -> Self {
        return Htsm{mat: self.mat.dot(&mat_to_transform.mat)};
    }

    pub fn rot_x(&self, theta: f64) -> Self {
        let theta = (theta * 3.1415) / 180.0;
        let rot_mat = ndarray::arr2(&[
            [1.0, 0.0, 0.0, 0.0],
            [0.0, theta.cos(), -theta.sin(), 0.0],
            [0.0, theta.sin(), theta.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0]]);
        return Htsm{mat: self.mat.dot(&rot_mat)};
    }

    pub fn rot_y(&self, theta: f64) -> Self {
        let theta = (theta * 3.1415) / 180.0;
        let rot_mat = ndarray::arr2(&[
            [theta.cos(), 0.0, theta.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-theta.sin(), 0.0, theta.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0]]);
        return Htsm{mat: self.mat.dot(&rot_mat)};
    }

    pub fn rot_z(&self, theta: f64) -> Self {
        let theta = (theta * 3.1415) / 180.0;
        let rot_mat = ndarray::arr2(&[
            [theta.cos(), -theta.sin(), 0.0, 0.0],
            [theta.sin(), theta.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0]]);
        return Htsm{mat: self.mat.dot(&rot_mat)};
    }

    pub fn trans(&self, x: f64, y: f64, z: f64) -> Self {
        let trans_mat = ndarray::arr2(&[
            [1.0, 0.0, 0.0, x],
            [0.0, 1.0, 0.0, y],
            [0.0, 0.0, 1.0, z],
            [0.0, 0.0, 0.0, 1.0]
        ]);

        return Htsm{mat: self.mat.dot(&trans_mat)};
    }
}

impl std::fmt::Display for Htsm {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Transformation Matrix : \n{:?}", self.mat);

        return Result::Ok(());
    }
}
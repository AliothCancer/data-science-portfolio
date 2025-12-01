#![allow(unused)]

struct MSZoning([&'static str;5]);
const MSZONING: MSZoning = MSZoning(["RH","FV","RM","RL","C (all)",]);

enum Street{Grvl,Pave,}

enum Alley{Pave,Grvl,}

enum LotShape{Reg,IR1,IR3,IR2,}

enum LandContour{Lvl,Low,HLS,Bnk,}

enum Utilities{AllPub,NoSeWa,}

enum LotConfig{CulDSac,Corner,Inside,FR3,FR2,}

enum LandSlope{Sev,Mod,Gtl,}

enum Neighborhood{NPkVill,SawyerW,Somerst,OldTown,NoRidge,Mitchel,BrDale,Blueste,MeadowV,Timber,Sawyer,IDOTRR,Blmngtn,BrkSide,ClearCr,SWISU,NAmes,NridgHt,NWAmes,CollgCr,Crawfor,StoneBr,Gilbert,Edwards,Veenker,}

enum Condition1{Artery,RRAn,RRAe,RRNn,Feedr,PosN,PosA,RRNe,Norm,}

enum Condition2{RRAe,RRNn,Norm,PosA,Feedr,PosN,RRAn,Artery,}

struct BldgType([&'static str;5]);
const BLDGTYPE: BldgType = BldgType(["Twnhs","Duplex","1Fam","2fmCon","TwnhsE",]);

struct HouseStyle([&'static str;8]);
const HOUSESTYLE: HouseStyle = HouseStyle(["1Story","2.5Fin","2Story","SLvl","1.5Unf","2.5Unf","SFoyer","1.5Fin",]);

enum RoofStyle{Hip,Gable,Flat,Shed,Mansard,Gambrel,}

struct RoofMatl([&'static str;8]);
const ROOFMATL: RoofMatl = RoofMatl(["WdShngl","Roll","CompShg","Tar&Grv","WdShake","ClyTile","Metal","Membran",]);

struct Exterior1st([&'static str;15]);
const EXTERIOR1ST: Exterior1st = Exterior1st(["CemntBd","AsphShn","VinylSd","Plywood","CBlock","WdShing","HdBoard","BrkFace","ImStucc","Stucco","Stone","BrkComm","Wd Sdng","MetalSd","AsbShng",]);

struct Exterior2nd([&'static str;16]);
const EXTERIOR2ND: Exterior2nd = Exterior2nd(["ImStucc","Wd Sdng","MetalSd","Plywood","AsbShng","Stucco","CmentBd","AsphShn","Brk Cmn","Other","Wd Shng","BrkFace","Stone","CBlock","HdBoard","VinylSd",]);

enum MasVnrType{None,Stone,BrkFace,BrkCmn,}

enum ExterQual{Fa,TA,Gd,Ex,}

enum ExterCond{Fa,Gd,TA,Ex,Po,}

enum Foundation{Slab,BrkTil,CBlock,PConc,Wood,Stone,}

enum BsmtQual{Ex,TA,Fa,Gd,}

enum BsmtCond{Po,Gd,TA,Fa,}

enum BsmtExposure{Mn,No,Gd,Av,}

enum BsmtFinType1{BLQ,Unf,Rec,LwQ,GLQ,ALQ,}

enum BsmtFinType2{BLQ,GLQ,Unf,ALQ,LwQ,Rec,}

enum Heating{GasA,Floor,Grav,GasW,Wall,OthW,}

enum HeatingQC{Po,Fa,Gd,TA,Ex,}

enum CentralAir{N,Y,}

enum Electrical{FuseF,SBrkr,FuseP,Mix,FuseA,}

enum KitchenQual{TA,Ex,Gd,Fa,}

enum Functional{Sev,Min1,Mod,Min2,Maj1,Typ,Maj2,}

enum FireplaceQu{Fa,Po,Gd,Ex,TA,}

struct GarageType([&'static str;6]);
const GARAGETYPE: GarageType = GarageType(["CarPort","Detchd","Basment","2Types","BuiltIn","Attchd",]);

enum GarageFinish{RFn,Unf,Fin,}

enum GarageQual{Gd,Ex,Po,TA,Fa,}

enum GarageCond{Gd,Ex,Po,TA,Fa,}

enum PavedDrive{Y,N,P,}

enum PoolQC{Fa,Ex,Gd,}

enum Fence{GdWo,GdPrv,MnPrv,MnWw,}

enum MiscFeature{Shed,TenC,Othr,Gar2,}

enum SaleType{CWD,Con,ConLD,ConLI,COD,Oth,ConLw,WD,New,}

enum SaleCondition{Abnorml,Normal,Family,Alloca,Partial,AdjLand,}


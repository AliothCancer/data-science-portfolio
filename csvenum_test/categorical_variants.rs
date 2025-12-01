struct MSZoning(Vec<&str>);
const MSZONING: MSZoning = Vec::from(["RM","RL","C (all)","FV","RH",]);

enum Street{Grvl,Pave,}

enum Alley{Grvl,Pave,}

enum LotShape{IR2,Reg,IR1,IR3,}

enum LandContour{Lvl,Bnk,Low,HLS,}

enum Utilities{NoSeWa,AllPub,}

enum LotConfig{FR2,FR3,CulDSac,Inside,Corner,}

enum LandSlope{Mod,Gtl,Sev,}

enum Neighborhood{Sawyer,Veenker,IDOTRR,Mitchel,NPkVill,Blmngtn,Somerst,ClearCr,OldTown,NridgHt,NWAmes,Blueste,MeadowV,BrDale,NoRidge,CollgCr,Timber,BrkSide,Gilbert,NAmes,SawyerW,Edwards,SWISU,StoneBr,Crawfor,}

enum Condition1{Norm,PosN,PosA,RRAe,Feedr,RRNn,Artery,RRAn,RRNe,}

enum Condition2{PosA,RRAe,Norm,RRNn,Feedr,PosN,RRAn,Artery,}

struct BldgType(Vec<&str>);
const BLDGTYPE: BldgType = Vec::from(["TwnhsE","Duplex","2fmCon","Twnhs","1Fam",]);

struct HouseStyle(Vec<&str>);
const HOUSESTYLE: HouseStyle = Vec::from(["2Story","SLvl","1Story","1.5Unf","SFoyer","2.5Fin","2.5Unf","1.5Fin",]);

enum RoofStyle{Mansard,Gable,Flat,Shed,Hip,Gambrel,}

struct RoofMatl(Vec<&str>);
const ROOFMATL: RoofMatl = Vec::from(["WdShake","CompShg","Metal","Tar&Grv","Membran","Roll","ClyTile","WdShngl",]);

struct Exterior1st(Vec<&str>);
const EXTERIOR1ST: Exterior1st = Vec::from(["AsphShn","WdShing","BrkComm","Stucco","Plywood","BrkFace","ImStucc","Wd Sdng","CBlock","MetalSd","AsbShng","HdBoard","CemntBd","Stone","VinylSd",]);

struct Exterior2nd(Vec<&str>);
const EXTERIOR2ND: Exterior2nd = Vec::from(["ImStucc","AsbShng","VinylSd","Wd Shng","Stucco","Wd Sdng","Other","Brk Cmn","AsphShn","Plywood","BrkFace","HdBoard","MetalSd","CmentBd","Stone","CBlock",]);

enum MasVnrType{None,BrkCmn,Stone,BrkFace,}

enum ExterQual{Gd,Ex,TA,Fa,}

enum ExterCond{Fa,Po,TA,Gd,Ex,}

enum Foundation{Wood,Slab,Stone,BrkTil,PConc,CBlock,}

enum BsmtQual{Ex,Fa,Gd,TA,}

enum BsmtCond{Fa,Po,TA,Gd,}

enum BsmtExposure{Gd,Av,Mn,No,}

enum BsmtFinType1{GLQ,Rec,BLQ,ALQ,LwQ,Unf,}

enum BsmtFinType2{Unf,Rec,LwQ,ALQ,GLQ,BLQ,}

enum Heating{Grav,OthW,GasW,Wall,Floor,GasA,}

enum HeatingQC{Po,Ex,TA,Fa,Gd,}

enum CentralAir{Y,N,}

enum Electrical{SBrkr,FuseF,FuseA,FuseP,Mix,}

enum KitchenQual{Ex,Fa,TA,Gd,}

enum Functional{Mod,Maj2,Maj1,Sev,Min2,Min1,Typ,}

enum FireplaceQu{Gd,Fa,Ex,TA,Po,}

struct GarageType(Vec<&str>);
const GARAGETYPE: GarageType = Vec::from(["2Types","Detchd","BuiltIn","Basment","Attchd","CarPort",]);

enum GarageFinish{RFn,Fin,Unf,}

enum GarageQual{Po,Gd,Ex,TA,Fa,}

enum GarageCond{Po,Gd,Ex,Fa,TA,}

enum PavedDrive{N,Y,P,}

enum PoolQC{Ex,Fa,Gd,}

enum Fence{MnWw,MnPrv,GdWo,GdPrv,}

enum MiscFeature{Shed,TenC,Othr,Gar2,}

enum SaleType{Oth,WD,ConLI,ConLD,ConLw,CWD,New,Con,COD,}

enum SaleCondition{Normal,AdjLand,Partial,Abnorml,Family,Alloca,}


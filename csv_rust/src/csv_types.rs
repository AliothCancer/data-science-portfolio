#![allow(unused)]

use serde::Deserialize;

#[derive(Debug,Deserialize)]
enum MSZoning{CSpaceOpenParenallCloseParen,}

#[derive(Debug,Deserialize)]
enum Street{Pave,Grvl,}

#[derive(Debug,Deserialize)]
enum Alley{Pave,Grvl,}

#[derive(Debug,Deserialize)]
enum LotShape{IR3,IR1,Reg,IR2,}

#[derive(Debug,Deserialize)]
enum LandContour{Low,Lvl,Bnk,HLS,}

#[derive(Debug,Deserialize)]
enum Utilities{NoSeWa,AllPub,}

#[derive(Debug,Deserialize)]
enum LotConfig{FR3,Corner,CulDSac,FR2,Inside,}

#[derive(Debug,Deserialize)]
enum LandSlope{Gtl,Mod,Sev,}

#[derive(Debug,Deserialize)]
enum Neighborhood{StoneBr,NPkVill,Blmngtn,SWISU,NridgHt,NoRidge,SawyerW,OldTown,Timber,Crawfor,NWAmes,BrDale,Blueste,BrkSide,Mitchel,Edwards,Gilbert,Somerst,Sawyer,MeadowV,NAmes,Veenker,CollgCr,IDOTRR,ClearCr,}

#[derive(Debug,Deserialize)]
enum Condition1{RRAn,RRNn,Norm,RRNe,PosN,Feedr,Artery,PosA,RRAe,}

#[derive(Debug,Deserialize)]
enum Condition2{RRAn,PosN,Feedr,PosA,RRNn,Artery,RRAe,Norm,}

#[derive(Debug,Deserialize)]
enum BldgType{TwofmCon,OneFam,}

#[derive(Debug,Deserialize)]
enum HouseStyle{OnePointFiveFin,TwoStory,OneStory,OnePointFiveUnf,TwoPointFiveFin,TwoPointFiveUnf,}

#[derive(Debug,Deserialize)]
enum RoofStyle{Flat,Mansard,Hip,Gable,Shed,Gambrel,}

#[derive(Debug,Deserialize)]
enum RoofMatl{TarAndGrv,}

#[derive(Debug,Deserialize)]
enum Exterior1st{WdSpaceSdng,}

#[derive(Debug,Deserialize)]
enum Exterior2nd{BrkSpaceCmn,WdSpaceShng,WdSpaceSdng,}

#[derive(Debug,Deserialize)]
enum MasVnrType{BrkCmn,Stone,None,BrkFace,}

#[derive(Debug,Deserialize)]
enum ExterQual{Ex,TA,Gd,Fa,}

#[derive(Debug,Deserialize)]
enum ExterCond{Gd,Ex,Po,TA,Fa,}

#[derive(Debug,Deserialize)]
enum Foundation{BrkTil,Wood,Slab,CBlock,PConc,Stone,}

#[derive(Debug,Deserialize)]
enum BsmtQual{Ex,Fa,Gd,TA,}

#[derive(Debug,Deserialize)]
enum BsmtCond{Gd,Fa,Po,TA,}

#[derive(Debug,Deserialize)]
enum BsmtExposure{Mn,Gd,Av,No,}

#[derive(Debug,Deserialize)]
enum BsmtFinType1{GLQ,ALQ,Rec,BLQ,Unf,LwQ,}

#[derive(Debug,Deserialize)]
enum BsmtFinType2{Unf,BLQ,Rec,LwQ,GLQ,ALQ,}

#[derive(Debug,Deserialize)]
enum Heating{Floor,GasA,OthW,Grav,GasW,Wall,}

#[derive(Debug,Deserialize)]
enum HeatingQC{TA,Fa,Ex,Po,Gd,}

#[derive(Debug,Deserialize)]
enum CentralAir{N,Y,}

#[derive(Debug,Deserialize)]
enum Electrical{Mix,SBrkr,FuseF,FuseA,FuseP,}

#[derive(Debug,Deserialize)]
enum KitchenQual{TA,Fa,Gd,Ex,}

#[derive(Debug,Deserialize)]
enum Functional{Min1,Maj2,Sev,Mod,Maj1,Typ,Min2,}

#[derive(Debug,Deserialize)]
enum FireplaceQu{TA,Fa,Ex,Gd,Po,}

#[derive(Debug,Deserialize)]
enum GarageType{TwoTypes,}

#[derive(Debug,Deserialize)]
enum GarageFinish{Unf,RFn,Fin,}

#[derive(Debug,Deserialize)]
enum GarageQual{Fa,TA,Po,Ex,Gd,}

#[derive(Debug,Deserialize)]
enum GarageCond{Ex,Fa,Po,TA,Gd,}

#[derive(Debug,Deserialize)]
enum PavedDrive{P,N,Y,}

#[derive(Debug,Deserialize)]
enum PoolQC{Fa,Ex,Gd,}

#[derive(Debug,Deserialize)]
enum Fence{MnWw,GdWo,MnPrv,GdPrv,}

#[derive(Debug,Deserialize)]
enum MiscFeature{TenC,Othr,Gar2,Shed,}

#[derive(Debug,Deserialize)]
enum SaleType{ConLI,CWD,ConLD,WD,Con,ConLw,COD,Oth,New,}

#[derive(Debug,Deserialize)]
enum SaleCondition{Partial,Abnorml,AdjLand,Normal,Family,Alloca,}

#[derive(Debug,Deserialize)]
struct SerdeCsvDeserializer{mszoning : MSZoning,street : Street,alley : Alley,lotshape : LotShape,landcontour : LandContour,utilities : Utilities,lotconfig : LotConfig,landslope : LandSlope,neighborhood : Neighborhood,condition1 : Condition1,condition2 : Condition2,bldgtype : BldgType,housestyle : HouseStyle,roofstyle : RoofStyle,roofmatl : RoofMatl,exterior1st : Exterior1st,exterior2nd : Exterior2nd,masvnrtype : MasVnrType,exterqual : ExterQual,extercond : ExterCond,foundation : Foundation,bsmtqual : BsmtQual,bsmtcond : BsmtCond,bsmtexposure : BsmtExposure,bsmtfintype1 : BsmtFinType1,bsmtfintype2 : BsmtFinType2,heating : Heating,heatingqc : HeatingQC,centralair : CentralAir,electrical : Electrical,kitchenqual : KitchenQual,functional : Functional,fireplacequ : FireplaceQu,garagetype : GarageType,garagefinish : GarageFinish,garagequal : GarageQual,garagecond : GarageCond,paveddrive : PavedDrive,poolqc : PoolQC,fence : Fence,miscfeature : MiscFeature,saletype : SaleType,salecondition : SaleCondition,}
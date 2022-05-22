port module Main exposing (main)

import Browser
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Json.Encode as Encode

{--
import Browser
import File exposing (File)
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Json.Decode as JD
import Task
--}

-- MAIN

main : Program () Model Msg
main =
    Browser.element
        { init = init
        , view = view
        , update = update
        , subscriptions = \_ -> Sub.none
        }

-- MODEL

type alias Model =
     { age : String
     , drivingExperience : String
     , vehicleMake : String
     , vehicleModel: String
     , vehicleYear : String
     }

init : () -> (Model, Cmd msg)
init _ =
  ({ age = "30"
   , drivingExperience = "10"
   , vehicleMake = "Škoda"
   , vehicleModel = "Superb"
   , vehicleYear = "2015"
   }
   ,
   Cmd.none)

modelToJsonString : Model -> String
modelToJsonString model =
  "{" ++
  "\"age\": " ++  model.age ++ ", " ++
  "\"driving_experience\": " ++  model.drivingExperience ++ ", " ++
  "\"vehicle_make\": \"" ++  model.vehicleMake ++ "\", " ++
  "\"vehicle_model\": \"" ++  model.vehicleModel ++ "\", " ++
  "\"vehicle_year\": " ++  model.vehicleYear ++
  "}"

-- UPDATE


type Msg
  = Age String
  | DrivingExperience String
  | VehicleMake String
  | VehicleModel String
  | VehicleYear String
  | IncorrectInput String
  | Send String


update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
  case msg of
    Age age ->
      ({ model | age = age }, Cmd.none)

    DrivingExperience drivingExperience ->
      ({ model | drivingExperience = drivingExperience }, Cmd.none)

    VehicleMake vehicleMake ->
      ({ model | vehicleMake = vehicleMake }, Cmd.none)

    VehicleModel vehicleModel ->
      ({ model | vehicleModel = vehicleModel }, Cmd.none)

    VehicleYear vehicleYear ->
      ({ model | vehicleYear = vehicleYear }, Cmd.none)
    
    IncorrectInput _ -> (model, Cmd.none)
    Send m -> (model, sendMessage m)

-- VIEW


view : Model -> Html Msg
view model =
  div []
    [ h2 [] ["Welcome to my Rust/Elm insurance quote calculation example." |> Html.text ]
    , h2 [] ["Here the user must fill the data for getting the insurance quote:" |> Html.text ]
    , div []
      [ viewInputText "number" "Age" model.age Age
      , viewInputText "number" "Driving Experience Years" model.drivingExperience DrivingExperience
      , viewInputText "text" "Vehicle Make" model.vehicleMake VehicleMake
      , viewInputText "text""Vehicle Model" model.vehicleModel VehicleModel
      , viewInputText "number" "Vehicle Year" model.vehicleYear VehicleYear
      , viewValidation model
      , button [ onClick (Send (modelToJsonString model)) ] [ text "Get insurance quote!" ]
      ]
    , h2 [] ["The pricing engine (in Rust, running on client by means of WebAssembly for this example ) will be shown here after pressing the button:" |> Html.text ]
    ]


viewInputText : String -> String -> String -> (String -> msg) -> Html msg
viewInputText t p v toMsg =
  input [ type_ t, placeholder p, value v, onInput toMsg ] []

viewValidation : Model -> Html msg
viewValidation model =
  case String.toInt model.age of
   Just age -> if age > 16 then
                 div [ style "color" "green" ] [ text "OK" ]
               else
                 div [ style "color" "red" ] [ text "Age must be greater than 16!" ]
   Nothing -> div [ style "color" "red" ] [ text "Bad age format." ]
   


-- PORTS


port sendMessage : String -> Cmd msg

{--
port messageReceiver : (String -> msg) -> Sub msg

-- SUBSCRIPTIONS

subscriptions : Model -> Sub Msg
subscriptions _ =
  messageReceiver Recv
--}

